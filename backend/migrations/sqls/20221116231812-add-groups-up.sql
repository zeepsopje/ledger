CREATE TABLE groups (
	id INT UNSIGNED AUTO_INCREMENT,
	name VARCHAR(255) NOT NULL,
	color ENUM('red', 'orange', 'yellow', 'green', 'blue', 'violet', 'pink'),
	PRIMARY KEY (id)
);
