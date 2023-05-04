/** A component that draws a large box around the main content and centres it on the page
 */
function PageContainerBoxLarge(props) {
    const { children, title } = props;

    return <div className="container-fluid vh-100 vw-100">
        <div className="row justify-content-center my-lg-4">
            <div className="col-lg-10 border rounded p-3">
        // Only display the title if it is available
                {
                    title ?
                        <h2 className="mx-auto text-center">{title}</h2>
                        : null
                }
                {children}
            </div>
        </div>
    </div>;
}
