const useState = React.useState;
const useEffect = React.useEffect;

/** The root component that is used to create pages
 */
function App(props) {
  const { children } = props;
  return <div>
    <Header />
    {children}
  </div>;
}
