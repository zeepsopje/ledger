<script>
	import { onMount } from 'svelte';
	import { getExpenses } from '@lib/ledger';
	import Expense from './Expense.svelte';

	let expenses = [];

	onMount(async () => {
		const res = await getExpenses();
		expenses = res.data;
	});
</script>

{#if expenses.length > 0}
	{#each expenses as {name, amount, created_at: date}}
		<Expense {name} {amount} {date} />
	{/each}
{:else}
	Loading...
{/if}
