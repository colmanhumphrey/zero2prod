CREATE TABLE subscription_tokens (
       subscription_token text NOT NULL,
       subscriber_id      UUID NOT NULL
           REFERENCES subscriptions(id),
       PRIMARY KEY (subscription_token)
);
