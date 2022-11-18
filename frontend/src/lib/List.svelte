<script>
	import { onMount } from 'svelte';
	import { getExpenses } from '@lib/ledger';

	let expenses = [];

	onMount(async () => {
		const res = await getExpenses();
		expenses = res.data;
	});
</script>

{#each expenses as {name, amount, created_at: date}}
	<div class="item">
	</div>
{/each}

<style>
</style>
