const Button = ReactBootstrap.Button;

const useState = React.useState;
const useEffect = React.useEffect;

function Calendar() {
  const test_events = [{
    start_time: new Date(1666217225862), // a date that spans two days, for testing
    duration: 2 * 60 * 60 * 1000,
    title: "Title",
    participants: "Participants"
  }];

  return <PageContainerBoxLarge title="Your Calendar">
    <Timetable events={test_events} initial_date={new Date()} />
  </PageContainerBoxLarge>;
}
