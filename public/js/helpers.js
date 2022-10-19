
// A fetch helper. `f` is short for `fetch`
function f(url, method, data) {
  const body = method === "GET" ? undefined : JSON.stringify(data);
  return fetch(url, {
    method: method,
    credentials: "same-origin", // only send cookies for same-origin requests
    headers: {
      "content-type": "application/json",
      accept: "application/json",
    },
    headers: {
      'Content-Type': 'application/json',
    },
    body
  });
}

// A react hook that calls a function when the window is resized
function useWindowResize(fun) {
  // Call when the DOM is initialised
  useEffect(() => {
    // Add event listener
    window.addEventListener("resize", fun);

    // Call the function when first loaded
    fun();

    // Remove event listener on cleanup
    return () => window.removeEventListener("resize", fun);
  }, []);
}
