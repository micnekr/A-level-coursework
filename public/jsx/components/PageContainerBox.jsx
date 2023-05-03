/** A component that draws a box around the main content and centres it on the page
 */
function PageContainerBox(props) {
    const { children, title } = props;

  return <div className="container-fluid vh-100 vw-100">
    <div className="row justify-content-center align-items-center h-100">
      <div className="col-8 border rounded p-3">
      // display the title if available
        {
          title ?
            <h2 className="mx-auto text-center">{title}</h2>
            : null
        }
        // Display the content
        {children}
      </div>
    </div>
  </div>;
}
