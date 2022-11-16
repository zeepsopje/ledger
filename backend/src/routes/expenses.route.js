import { Router } from 'express';
import db from '../lib/db';

const r = Router();

r.get('/', (_, res, next) => {
	db.query('SELECT * FROM expenses')
		.then(([data]) => {
			res.json(data);
		})
		.catch(err => {
			next(err);
		});
});

r.post('/', (req, res, next) => {
	db.query('INSERT INTO expenses SET ?', {
		name: req.body.name,
		amount: req.body.amount,
	})
		.then(() => {
			res.sendStatus(201);
		})
		.catch(err => next(err));
});

export default r;
