import mysql from 'mysql2/promise';

const {
	DB_NAME,
	DB_USER,
	DB_PASS,
	DB_HOST = 'localhost',
} = process.env;

const conn = mysql.createPool({
	database: DB_NAME,
	user: DB_USER,
	password: DB_PASS,
	host: DB_HOST,
});

export default conn;
