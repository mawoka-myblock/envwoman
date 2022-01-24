<!--
    https://svelte.dev/repl/5c54587a39f9420d850b6b09b3c3d601?version=3.23.0
-->

<script>
	import { onMount } from 'svelte';
	import steps from './steps.js';

	let index = 0;
	let current;
	let currentEditorIndex = 0;
	let currentEditorCommand;
	let lines = [];
	let selections = [];
	let transcription;

	let nextTimer, typeTimer, transitionTimer, editorCommandTimer;

	onMount(() => {
		nextTimer = next();
	});

	function next() {
		current = { ...steps[index] };

		if (current.action == 'command' || current.action == 'editor') {
			current.typed = '';
			current.index = 0;
			typeTimer = setTimeout(type, 0);
		} else if (current.action == 'wait') {
			index += 1;
			nextTimer = setTimeout(next, current.delay || 1000);
		}
	}

	function type() {
		const char = current.command[current.index];
		current = { ...current, typed: current.typed + char };
		transcription = current.transcription || '';

		current.index += 1;

		if (current.index < current.command.length) {
			typeTimer = setTimeout(type, 50);
		} else {
			if (current.action == 'editor') {
				transitionTimer = setTimeout(openEditor, 1000);
			} else {
				scheduleTransition();
			}
		}
	}

	function scheduleTransition() {
		const delay = transcription ? 1500 : 300;
		transitionTimer = setTimeout(transition, delay);
	}

	function openEditor() {
		current.lines = current.content.split('\n');

		transcription = '';
		current.showEditor = true;
		current.selections = [];

		editorCommandTimer = setTimeout(nextEditorCommand, 1000);
	}

	function transition() {
		index += 1;
		lines = [...lines, current];
		current = null;
		transcription = '';

		nextTimer = setTimeout(next, 200);
	}

	function nextEditorCommand() {
		currentEditorCommand = current.steps[currentEditorIndex];
		currentEditorIndex += 1;
		transcription = currentEditorCommand.transcription;

		if (currentEditorCommand.action == 'close') {
			// editorCommandTimer = setTimeout(closeEditor, currentEditorCommand.delay)
		} else if (currentEditorCommand.action == 'select') {
			const { from, to, style } = currentEditorCommand;
			const selection = { from, to, style };

			selections.push(selection);
			selections = selections;
			current = current;

			console.log(selections);
			editorCommandTimer = setTimeout(nextEditorCommand, currentEditorCommand.delay);
		}
	}

	function isSelected(line, char) {
		if (!selections) return false;

		console.log({ line, char }, selections);

		selections.forEach(({ from, to }) => {
			debugger;
			if (from.line >= line && char >= from.char && to.line >= line && to.char >= char)
				return true;
		});

		return false;
	}

	function closeEditor() {
		current.showEditor = false;
		selections = [];
		scheduleTransition();
	}

	function reset() {
		clearTimeout(nextTimer);
		clearTimeout(typeTimer);
		clearTimeout(transitionTimer);
		clearTimeout(editorCommandTimer);

		current = null;
		index = 0;
		lines = [];
		transcription = null;
		selections = [];

		currentEditorIndex = 0;
		currentEditorCommand = null;

		nextTimer = setTimeout(next, 1000);
	}
</script>

<!--
<pre>
  lines = {JSON.stringify(lines, null, 2)}
</pre>
<pre>
	current = {JSON.stringify(current, null, 2)}
</pre>
-->

<!-- <svelte:window on:click={reset} /> -->

<section class="terminal drop-shadow-2xl">
	<div class="bar">
		<svg xmlns="http://www.w3.org/2000/svg" width="54" height="14" viewBox="0 0 54 14">
			<g fill="none" fill-rule="evenodd" transform="translate(1 1)">
				<circle cx="6" cy="6" r="6" fill="#FF5F56" stroke="#E0443E" stroke-width=".5" />
				<circle cx="26" cy="6" r="6" fill="#FFBD2E" stroke="#DEA123" stroke-width=".5" />
				<circle cx="46" cy="6" r="6" fill="#27C93F" stroke="#1AAB29" stroke-width=".5" />
			</g>
		</svg>
	</div>
	<div class="absolute top-6 right-0 h-16 w-16 hover:cursor-pointer" on:click={reset}>
		<svg
			width="24"
			height="24"
			viewBox="0 0 24 24"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
		>
			<path
				d="M13.1459 11.0499L12.9716 9.05752L15.3462 8.84977C14.4471 7.98322 13.2242 7.4503 11.8769 7.4503C9.11547 7.4503 6.87689 9.68888 6.87689 12.4503C6.87689 15.2117 9.11547 17.4503 11.8769 17.4503C13.6977 17.4503 15.2911 16.4771 16.1654 15.0224L18.1682 15.5231C17.0301 17.8487 14.6405 19.4503 11.8769 19.4503C8.0109 19.4503 4.87689 16.3163 4.87689 12.4503C4.87689 8.58431 8.0109 5.4503 11.8769 5.4503C13.8233 5.4503 15.5842 6.24474 16.853 7.52706L16.6078 4.72412L18.6002 4.5498L19.1231 10.527L13.1459 11.0499Z"
				fill="currentColor"
			/>
		</svg>
	</div>

	{#each lines as line}
		<p>
			<span class="prompt">&gt;</span>
			{line.command}
		</p>

		{#if line.output}
			<p class="output">
				{#each line.output.split('\n') as outputLine}
					{outputLine}<br />
				{/each}
			</p>
		{/if}
	{/each}

	{#if current && (current.action == 'command' || current.action == 'editor')}
		<p>
			<span class="prompt">&gt;</span>
			{current.typed}
		</p>
	{/if}

	{#if current && current.action == 'editor' && current.showEditor}
		<div class="editor">
			{#each current.lines as line, lineIndex}
				{#if current.lineNumbers}<span class="line-number">{lineIndex + 1}</span>{/if}
				{#each line.split() as char, charIndex}
					<span class:highlight={isSelected(lineIndex, charIndex)}>{char}</span>
				{/each}<br />
			{/each}<span class="cursor" />
		</div>
	{/if}

	<div class="transcription-wrapper">
		<div class="transcription" class:visible={!!transcription}>
			{transcription}
		</div>
	</div>
</section>

<style>
	/* :global(body) {
		background: white;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
	} */

	.terminal {
		/*position: relative;*/
		font-family: monospace;
		font-size: 1.5rem;
		background: #333;
		padding: 0.7rem 1rem;
		margin: 5rem;
		border-radius: 0.5rem;
		color: #b8b8b8;
		/*box-shadow: 1px 1px #ccc;*/
		min-height: 25rem;
		min-width: 35em;
		overflow: hidden;
	}

	.terminal .bar {
		margin-bottom: 1rem;
	}

	.transcription-wrapper {
		position: absolute;
		bottom: 0px;
		width: 100%;
		text-align: center;
		margin: 1.8rem 0;
	}

	.editor {
		position: absolute;
		top: 3.5rem;
		background: #333;
		width: -webkit-fill-available;
		height: -webkit-fill-available;
		margin-left: -1rem;
		padding: 0 1rem;
	}

	.editor .line-number {
		color: #ffbd2e;
		font-weight: bold;
		display: inline-block;
		margin-right: 0.5rem;
	}

	.editor .highlight {
		background: white;
		color: black;
	}

	.transcription {
		display: inline-block;
		font-family: sans-serif;
		font-size: 0.9rem;
		background: #ccc;
		padding: 0.5em;
		border-radius: 0.1em;
		color: #222;
		top: 5rem;
		position: relative;
		transition: all 0.3s ease-out;
	}

	.transcription.visible {
		top: 0;
	}

	.prompt {
		color: #27c93f;
		font-weight: bold;
	}

	p {
		margin: 0.2rem 0;
	}
	p.output {
		font-weight: normal;
	}
</style>
