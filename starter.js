async function run() {
	const main = import('./domik.js').then(
		({default: init, main}) =>
		init()
	)
}

console.log('--> activated starter.js')
run();
