import { useRef, useState } from 'react'
// import reactLogo from './assets/react.svg'
// import viteLogo from '/vite.svg'
import './App.css'

import EquationInput from './EquationInput'
import Graph from './Graph';
import { Point } from './utils';

function App() {
	const [equations, setEquations] = useState([
		"",
	]);

  return (
	<>
		<div className="split left">
			<EquationInput 
				equations={equations}
				setEquations={setEquations}
			/>
		</div>
		<div className="split right">
			<Graph equations={equations} setEquations={setEquations} />
		</div>
    </>
  )
}

export default App;