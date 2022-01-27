<script>
	import FakeTerminal from "$lib/FakeTerminal.svelte";
	import Footer from "$lib/Footer.svelte";
</script>

<svelte:head>
	<title>Envwoman - dotenv synchroniser</title>
	<meta name="description" content="An application to sync your dotenv-files with others pcs and your server." />
</svelte:head>


<div class="pt-12 h-full pb-8">
	<section class="section">
		<div class="container pt-12 text-center">
			<h1 class="sm:text-8xl text-6xl mt-6 marck-script">Envwoman</h1>
			<p class="text-xl mt-4">Your .env synchroniser!</p>
		</div>
	</section>
	
	
	<div class="pt-12 flex justify-center">
		<div class="hidden md:block">
			<FakeTerminal />
		</div>
		<div class="md:hidden text-center">
			<p>On the desktop, you would get a cool demo here...</p>
			<p>But that doesn't matter! Envwoman is still awesome, although this website clearly isn't!</p>
		</div>
		
	</div>

	<div id="install">
		<h1 class="text-center text-5xl marck-script px-4">Installation</h1>
		<p class="text-center text-lg">Just grab the most recent version from <a class="underline hover:text-black" href="https://github.com/mawoka-myblock/envwoman/releases">GitHub</a> and run it. If you want to compile it for yourself,
		follow the instructions on GitHub.</p>
	</div>
</div>
<Footer />
