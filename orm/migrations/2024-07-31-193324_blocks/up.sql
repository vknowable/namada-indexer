-- Your SQL goes here

CREATE TABLE blocks (
    id VARCHAR(64) PRIMARY KEY,
    height INT,
    epoch INT,
    time VARCHAR,
    proposer_address VARCHAR(40),
    wrapper_txs VARCHAR[],
    inner_txs VARCHAR[],
    signatures VARCHAR[]
);
