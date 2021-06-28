create table wallet
(
	wallet_id serial not null
		constraint wallet_pk
			primary key,
	height varchar(50),
    r#type varchar(50),
    txhash varchar(50),
    r#key varchar(200),
    amount varchar(50),
    owner varchar(50),
    gas_used varchar(50)
);