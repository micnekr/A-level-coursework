const Alert = ReactBootstrap.Alert;
/** A component that displays an error message only if there is an error
 */
function ErrorMessage(props) {
  // em is short for "error message"
  let { em } = props;

  // If there is nothing to display, do not display an error box
  if (em === "") return null;

  return <Alert variant="danger" className="mt-3 mb-3">
    {em}
  </Alert>;
}
