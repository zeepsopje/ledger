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

export default r;
