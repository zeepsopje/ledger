import axios from 'axios';

const client = axios.create({
	baseURL: 'http://localhost:3000',
	headers: {
		'Accept': 'application/json',
		'Content-Type': 'application/json',
	}
});

export function getExpenses() {
	return client.get('/expenses');
}

export function addExpense(expense) {
	return client.post('/expenses', expense);
}
