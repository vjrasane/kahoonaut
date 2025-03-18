<script lang="ts">
	import { getContext } from 'svelte';
	export let href = '#';
	export let onClick: (e: MouseEvent) => void;
	export let disabled: boolean | undefined = false;

	// Retrieve the closeMenu function from context
	const closeMenu = getContext<() => void>('closeMenu');

	function handleClick(e: MouseEvent) {
		onClick(e);
		closeMenu();
	}

	$: disabledClass = disabled ? 'menu-disabled' : '';
</script>

<li class={disabledClass}>
	<a {href} on:click={handleClick}>
		<slot />
	</a>
</li>
