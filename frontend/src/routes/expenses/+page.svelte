<script>
	import { addExpense } from '@lib/ledger';
	import Expenses from '@lib/Expenses.svelte';

	let shuffle = 0;

	function handleExpenseForm({ target }) {
		const form = new FormData(target);
		const expense = {
			name: form.get('name'),
			amount: form.get('amount'),
		};

		addExpense(expense)
			.then(() => {
				shuffle++
			});
	}
</script>

<h1>Expenses</h1>
{#key shuffle}
	<Expenses />
{/key}

<form on:submit|preventDefault={handleExpenseForm}>
	<input name="name" placeholder="Name" type="text" />
	<input name="amount" placeholder="Amount" type="number" />
	<input type="submit" />
</form>
