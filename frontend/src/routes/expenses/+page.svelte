<script>
	import ledger from '@lib/ledger';
	import Expense  from '@lib/Expense.svelte';

	let expenses = ledger.getExpenses();
</script>

<h1>Expenses</h1>

{#await expenses}
{:then expenses}
	{#each expenses as {name, amount, created_at: date}}
		<Expense {name} {date} {amount} />
	{/each}
{/await}
