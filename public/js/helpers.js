
// A fetch helper. `f` is short for `fetch`
function f(url, method, data) {
  const body = method === "GET" ? undefined : JSON.stringify(data);
  return fetch(url, {
    method: method,
    credentials: "same-origin", // only send cookies for same-origin requests
    // We are using JSON
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

/** A function that converts a number to its corresponding ordinal number, e.g. 1 to 1st
*/
function to_ordinal(number) {
  // Convert to string
  const str = number + "";
  // get last digit
  const last_digit = str[str.length - 1];
  const suffix =
    last_digit === "1" ? "st"
      : last_digit === "2" ? "nd"
        : last_digit === "3" ? "rd" : "th";
  return str + suffix;
}
