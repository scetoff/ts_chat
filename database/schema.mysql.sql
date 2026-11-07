-- TSchat legacy MySQL schema, transcribed from function.vb::InitilizeDatabase.
-- Run this manually ONLY against a brand-new, disposable test database.
-- Do not run against production or a database containing real user data.
--
-- The legacy client used latin1 and these exact table/column names.
-- No existing table is altered by this script.

CREATE TABLE IF NOT EXISTS `tb_chat` (
    `id` INT(11) NOT NULL,
    `chat` LONGTEXT NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE IF NOT EXISTS `tb_pm` (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `username` VARCHAR(200) NOT NULL,
    `fromuser` VARCHAR(200) NOT NULL,
    `subject` VARCHAR(300) NOT NULL,
    `message` TEXT NOT NULL,
    `isread` INT(11) NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

CREATE TABLE IF NOT EXISTS `tb_users` (
    `id` INT(11) NOT NULL AUTO_INCREMENT,
    `username` VARCHAR(200) NOT NULL,
    `password` VARCHAR(200) NOT NULL,
    `email` VARCHAR(200) NOT NULL,
    `name` VARCHAR(200) NOT NULL,
    `dp` LONGTEXT NOT NULL,
    `mobile` VARCHAR(50) NOT NULL,
    `chatcolor` VARCHAR(100) NOT NULL,
    `chatfont` VARCHAR(100) NOT NULL,
    `chatfontsize` INT(11) NOT NULL,
    `chatfontstyle` VARCHAR(40) NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

INSERT IGNORE INTO `tb_chat` (`id`, `chat`)
VALUES (1, 'Добро пожаловать в Таверну!');
