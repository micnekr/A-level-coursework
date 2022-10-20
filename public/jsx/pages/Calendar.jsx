const Button = ReactBootstrap.Button;
const Pagination = ReactBootstrap.Pagination;

const useState = React.useState;
const useEffect = React.useEffect;

function Calendar() {
  const [start_of_week_date, set_start_of_week_date] = useState(
    dayjs(new Date()).startOf("week").add(1, "day")
  ); // Their week starts on a Sunday, so we add 1 to make it a Monday
  const test_events = [{
    start_time: 1666217225, // a date that spans two days, for testing
    duration: 2 * 60 * 60,
    title: "Title",
    participants: "Participants"
  }];


  return <PageContainerBoxLarge title="Your Calendar">
    <Pagination className="container">
      <div className="row justify-content-center w-100 gx-0">
        <Pagination.Prev className="col-auto" onClick={() => {
          // Go to the previous week
          set_start_of_week_date(start_of_week_date.subtract(1, "w"));
        }} />

        <Pagination.Item className="col-auto">{display_day_and_date(start_of_week_date)}-{display_day_and_date(start_of_week_date.add(6, "d"))}</Pagination.Item>

        <Pagination.Next className="col-auto" onClick={() => {
          // Go to the next week
          set_start_of_week_date(start_of_week_date.add(1, "w"));
        }} />
      </div>
    </Pagination>
    <Timetable events={test_events} start_of_week_date={start_of_week_date} />
  </PageContainerBoxLarge>;
}
