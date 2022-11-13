import 'dotenv/config';
import express from 'express';
import cors from 'cors';
import * as routes from './routes';

const app = express();

// App-wide middleware
app.use(express.json());
app.use(cors({ origin: '*' }));

// Routes
app.use('/expenses', routes.expenses);

export default app;
