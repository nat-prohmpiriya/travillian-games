-- Remove Chief troops
DELETE FROM troop_definitions WHERE troop_type IN ('royal_advisor', 'harbor_master', 'elder_chief');
