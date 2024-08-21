import { useEffect, useRef, useState } from 'react'
// import reactLogo from './assets/react.svg'
// import viteLogo from '/vite.svg'
import './App.css'

import EquationInput from './EquationInput'
import Graph from './Graph';

import init, { greet } from "./pkg/wasm_graph_calc.js"

function App() {
	const [equations, setEquations] = useState([
		"",
	]);

	const [wasmLoaded, setWasmLoaded] = useState(false);

	// load wasm
	useEffect(() => {
		init().then(() => {
			setWasmLoaded(true)
		})
	})

  return (
	<>
		<div className="split left">
			<EquationInput 
				equations={equations}
				setEquations={setEquations}
			/>
		</div>
		<div className="split right">
			<Graph
				equations={equations}
				setEquations={setEquations}

				wasmLoaded={wasmLoaded}
			/>
		</div>
    </>
  )
}

export default App;