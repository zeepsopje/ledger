const baseUrl = 'http://localhost:3000';

async function rawGet(route = '/') {
	const res = await fetch(baseUrl + route, {
		method: 'GET',
	});

	return await res.json();
}

async function getExpenses() {
	return await rawGet('/expenses');
}

export default {
	getExpenses
};
