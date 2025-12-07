package firebase

import (
	"context"
	"log"

	firebase "firebase.google.com/go/v4"
	"firebase.google.com/go/v4/auth"
	"google.golang.org/api/option"
)

type Client struct {
	App  *firebase.App
	Auth *auth.Client
}

func NewClient(credentialsPath string) (*Client, error) {
	ctx := context.Background()
	var opts []option.ClientOption

	if credentialsPath != "" {
		opts = append(opts, option.WithCredentialsFile(credentialsPath))
	}
	// If credentialsPath is empty, it attempts to use GOOGLE_APPLICATION_CREDENTIALS env var

	app, err := firebase.NewApp(ctx, nil, opts...)
	if err != nil {
		return nil, err
	}

	authClient, err := app.Auth(ctx)
	if err != nil {
		return nil, err
	}

	log.Println("Firebase Admin SDK initialized")
	return &Client{
		App:  app,
		Auth: authClient,
	}, nil
}

func (c *Client) VerifyToken(ctx context.Context, idToken string) (*auth.Token, error) {
	return c.Auth.VerifyIDToken(ctx, idToken)
}
