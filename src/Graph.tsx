import { useEffect } from "react";

function Graph({equations, setEquations}: {equations: any, setEquations: any}) {
	useEffect(() => {
		console.log(equations);
	}, [equations])

	return (
		
	);
}

export default Graph;