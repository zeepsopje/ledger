<script>
	import Date from './Date.svelte';

	export let name;
	export let amount;
	export let date;
</script>

<div class="expense">
	<Date value={date} />
	{name}
	{amount}
</div>

<style lang="scss">
	.expense {
		display: flex;
		align-items: center;
		border: solid 1px #ddd;
		padding: 20px;
	}
</style>
