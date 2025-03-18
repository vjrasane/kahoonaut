<script lang="ts">
	import { setContext } from 'svelte';

	// Track the open state of the menu
	let menuOpen = false;

	// Toggle menu open/close when clicking the icon
	function toggleMenu() {
		menuOpen = !menuOpen;
	}

	// Close the menu
	function closeMenu() {
		menuOpen = false;
	}
	// Provide closeMenu to descendant components via Svelte context
	setContext('closeMenu', closeMenu);

	// Custom Svelte action to detect clicks outside the node
	function clickOutside(node: HTMLElement) {
		const handleClick = (event: MouseEvent) => {
			if (!node.contains(event.target as Node)) {
				closeMenu();
			}
		};

		document.addEventListener('click', handleClick, true);

		return {
			destroy() {
				document.removeEventListener('click', handleClick, true);
			}
		};
	}
</script>

<!-- The container uses 'relative' so that the absolute menu is positioned correctly -->
<div class="relative" use:clickOutside>
	<!-- Hamburger icon button -->
	<button aria-label="menu" on:click={toggleMenu} class="btn btn-ghost btn-circle">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="h-6 w-6"
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M4 6h16M4 12h16M4 18h16"
			/>
		</svg>
	</button>

	<!-- Menu items (dropdown style using DaisyUI classes) -->
	{#if menuOpen}
		<ul class="menu bg-base-200 gap-2 rounded-box absolute right-0 w-48 z-100">
			<slot />
		</ul>
	{/if}
</div>
