import { Router } from 'express';

const r = Router();

r.use('/', (_, res) => {
	res.json("we\'re getting expenses");
});

export default r;
