<script lang="ts">
	import '@fortawesome/fontawesome-free/css/all.css';
	import Prism from 'prismjs';
	import 'prismjs/plugins/toolbar/prism-toolbar';
	import 'prismjs/plugins/copy-to-clipboard/prism-copy-to-clipboard';
	import 'prismjs/components/prism-json';
	import 'prismjs/components/prism-yaml';
	import 'prismjs/components/prism-bash';
	import 'prismjs/components/prism-powershell';
	import 'prismjs/themes/prism-okaidia.css';
	import { onMount } from 'svelte';
	import HeadComponent from '$lib/HeadComponent.svelte';
	import mentions from './mentions.json';

	interface Mention {
		title: string;
		contents: string[];
		links: { text: string; url: string }[];
	}

	interface PrismaRegisterButtonContext {
		element: { parentNode: HTMLElement | null };
	}

	let randomOrderMentions: Mention[] = [];

	let installUsing = 'manually';

	const handleAnchorClick = (evt: Event) => {
		evt.preventDefault();
		const link = evt.currentTarget as HTMLAnchorElement;
		const anchorId = new URL(link.href).hash.replace('#', '');
		const anchor = document.getElementById(anchorId);
		window.scrollTo({
			top: anchor?.offsetTop,
			behavior: 'smooth'
		});
	};

	const onInstallUsingChange = (evt: Event) => {
		const select = evt.currentTarget as HTMLSelectElement;
		installUsing = select.value;
	};

	onMount(() => {
		randomOrderMentions = mentions.sort(() => 0.5 - Math.random());
		Prism.plugins.toolbar.registerButton(
			'fullscreen-code',
			function (ctx: PrismaRegisterButtonContext) {
				const button = document.createElement('button');
				button.innerHTML = '🔍';
				button.addEventListener('click', function () {
					ctx.element.parentNode?.requestFullscreen();
				});

				return button;
			}
		);

		Prism.highlightAll();
	});
</script>

<HeadComponent
	data={{
		title: 'opendeck-ajazz',
		description: 'Linux software for your Ajazz stream controller devices.'
	}}
/>

<div id="start" class="hero bg-base-200 min-h-screen">
	<div class="hero-content text-center">
		<div class="max-w-md">
			<img src="/logo.svg" alt="opendeck-ajazz logo" class="m-5 mx-auto w-64" />
			<h1 class="text-5xl font-bold">opendeck-ajazz</h1>
			<p class="py-6">Linux software for your Ajazz stream controller devices.</p>
			<div role="alert" class="alert alert-info mb-5">
				<i class="fa fa-info" aria-hidden="true"></i>
				<span
					>The <a href="https://github.com/nekename/OpenDeck">OG OpenDeck</a> supports Ajazz devices
					via plugins.</span
				>
			</div>
			<a href="#install" on:click={handleAnchorClick}
				><button class="btn btn-primary">Get Started</button></a
			>
		</div>
	</div>
</div>
<div id="install" class="hero bg-base-200 min-h-screen">
	<div class="hero-content text-center">
		<div class="max-w-md">
			<h1 class="text-5xl font-bold">Install ⚡</h1>
			<p class="py-6">Install opendeck-ajazz using ...</p>
			<select on:input={onInstallUsingChange} class="select select-bordered mb-5">
				<option value="manually">manually</option>
			</select>
			<div class={installUsing === 'manually' ? '' : 'hidden'}>
				<p class="mb-5">
					Download the latest release from the <a class="text-secondary" href="/download"
						>releases page</a
					>.
				</p>
			</div>
			<div class={installUsing === 'curl' ? '' : 'hidden'}>
				<pre><code
						class="language-bash"
						data-toolbar-order="copy-to-clipboard"
						data-prismjs-copy="📋">curl -sSL https://opendeck.mwco.app/install.sh | bash</code
					></pre>
			</div>
			<p>
				<a href="#honorable-mentions" on:click={handleAnchorClick}
					><button class="btn btn-primary mt-5">Honorable mentions</button></a
				>
			</p>
		</div>
	</div>
</div>
<div id="honorable-mentions" class="hero bg-base-200 min-h-screen">
	<div class="hero-content text-center">
		<div class="max-w-md">
			<h1 class="text-5xl font-bold">Honorable mentions 🥰</h1>
			<p class="py-6">These projects helped make opendeck-ajazz possible:</p>
			{#each randomOrderMentions as mention, idx}
				{#if idx > 0}
					<div class="my-4"></div>
				{/if}
				<div class="card bg-base-100 mx-auto w-96 shadow-sm">
					<div class="card-body">
						<h2 class="card-title">
							{mention.title}
						</h2>
						{#each mention.contents as content}
							<p>{content}</p>
						{/each}
						<div class="card-actions justify-end">
							{#each mention.links as link}
								<a class="badge badge-outline" href={link.url}>{link.text}</a>
							{/each}
						</div>
					</div>
				</div>
			{/each}
			<p>
				<a href="#get-involved" on:click={handleAnchorClick}
					><button class="btn btn-primary mt-5">Get involved</button></a
				>
			</p>
		</div>
	</div>
</div>
<div id="get-involved" class="hero bg-base-200 min-h-screen">
	<div class="hero-content text-center">
		<div class="max-w-md">
			<h1 class="text-5xl font-bold">Get involved 📦</h1>
			<p class="py-6">opendeck-ajazz is open-source and we welcome contributions.</p>
			<p>
				View the <a class="text-secondary" href="https://github.com/mistweaverco/opendeck-ajazz"
					>code</a
				>, and/or check out the
				<a class="text-secondary" href="/docs">docs</a>.
			</p>
		</div>
	</div>
</div>
