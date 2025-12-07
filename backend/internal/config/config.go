package config

import (
	"log"

	"github.com/spf13/viper"
)

type Config struct {
	App      AppConfig
	Postgres PostgresConfig
	Redis    RedisConfig
	OTEL     OTELConfig
	JWT      JWTConfig
	Firebase FirebaseConfig
}

type AppConfig struct {
	Env      string `mapstructure:"APP_ENV"`
	Debug    bool   `mapstructure:"APP_DEBUG"`
	LogLevel string `mapstructure:"LOG_LEVEL"`
	Port     string `mapstructure:"PORT"`
}

type PostgresConfig struct {
	Host               string `mapstructure:"POSTGRES_HOST"`
	Port               string `mapstructure:"POSTGRES_PORT"`
	User               string `mapstructure:"POSTGRES_USER"`
	Password           string `mapstructure:"POSTGRES_PASSWORD"`
	DBName             string `mapstructure:"POSTGRES_DB"`
	SSLMode            string `mapstructure:"POSTGRES_SSLMODE"`
	MaxConnections     int    `mapstructure:"POSTGRES_MAX_CONNECTIONS"`
	MaxIdleConnections int    `mapstructure:"POSTGRES_MAX_IDLE_CONNECTIONS"`
}

type RedisConfig struct {
	Host       string `mapstructure:"REDIS_HOST"`
	Port       string `mapstructure:"REDIS_PORT"`
	Password   string `mapstructure:"REDIS_PASSWORD"`
	DB         int    `mapstructure:"REDIS_DB"`
	MaxRetries int    `mapstructure:"REDIS_MAX_RETRIES"`
	PoolSize   int    `mapstructure:"REDIS_POOL_SIZE"`
}

type OTELConfig struct {
	Endpoint    string `mapstructure:"OTEL_EXPORTER_OTLP_ENDPOINT"`
	Protocol    string `mapstructure:"OTEL_EXPORTER_OTLP_PROTOCOL"`
	ServiceName string `mapstructure:"OTEL_SERVICE_NAME"`
}

type JWTConfig struct {
	Secret             string `mapstructure:"JWT_SECRET"`
	AccessTokenExpiry  string `mapstructure:"JWT_ACCESS_TOKEN_EXPIRY"`
	RefreshTokenExpiry string `mapstructure:"JWT_REFRESH_TOKEN_EXPIRY"`
}

type FirebaseConfig struct {
	CredentialsPath string
}

func Load() (*Config, error) {
	viper.SetConfigFile(".env")
	viper.AutomaticEnv()

	// Defaults matching .env.example logic where appropriate
	viper.SetDefault("APP_ENV", "development")
	viper.SetDefault("APP_DEBUG", true)
	viper.SetDefault("LOG_LEVEL", "debug")
	viper.SetDefault("PORT", "8080")

	viper.SetDefault("POSTGRES_PORT", "5432")
	viper.SetDefault("POSTGRES_USER", "postgres")
	viper.SetDefault("POSTGRES_SSLMODE", "disable")
	viper.SetDefault("POSTGRES_MAX_CONNECTIONS", 100)
	viper.SetDefault("POSTGRES_MAX_IDLE_CONNECTIONS", 10)

	viper.SetDefault("REDIS_PORT", "6379")
	viper.SetDefault("REDIS_DB", 0)
	viper.SetDefault("REDIS_MAX_RETRIES", 3)
	viper.SetDefault("REDIS_POOL_SIZE", 100)

	viper.SetDefault("OTEL_EXPORTER_OTLP_PROTOCOL", "grpc")
	viper.SetDefault("POSTGRES_DB", "tusk_horn")
	viper.SetDefault("OTEL_SERVICE_NAME", "tusk-horn")

	// Attempt to read .env, but don't fail if missing (rely on env vars)
	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			return nil, err
		}
		log.Println("No .env file found, using environment variables")
	}

	var cfg Config
	if err := viper.Unmarshal(&cfg); err != nil {
		return nil, err
	}
	// Manual struct binding for sections if viper unmarshal root fails for nested without keys
	// But viper.Unmarshal(&cfg) generally maps if structure matches.
	// We bind explicitly to be safe as previously decided, OR rely on mapstructure tags if we added them.
	// Since I added mapstructure tags above, viper.Unmarshal(&cfg) should work for flat envs IF viper is configured correctly.
	// However, standard viper unmarshal from flat envs to nested struct requires some delimiter magic or manual unmarshal.
	// Let's do manual unmarshal for safety.

	if err := viper.Unmarshal(&cfg.App); err != nil {
		return nil, err
	}
	if err := viper.Unmarshal(&cfg.Postgres); err != nil {
		return nil, err
	}
	if err := viper.Unmarshal(&cfg.Redis); err != nil {
		return nil, err
	}
	if err := viper.Unmarshal(&cfg.OTEL); err != nil {
		return nil, err
	}
	if err := viper.Unmarshal(&cfg.JWT); err != nil {
		return nil, err
	}

	// Default port fix
	if cfg.App.Port == "" {
		cfg.App.Port = "8080"
	}

	return &cfg, nil
}
