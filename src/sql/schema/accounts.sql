CREATE TABLE accounts (
    id VARCHAR(36) NOT NULL DEFAULT (UUID()),
    username VARCHAR(45) NOT NULL,
    password VARCHAR(300) NOT NULL,
    joined DATETIME DEFAULT (NOW()),
    updated DATETIME,

    PRIMARY KEY (id)
);