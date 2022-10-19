/** A component that displays an event within a timetable
 */
function TimetableEvent(props) {
  const { container_ref, event } = props;
  const { start_time, end_time, duration, title, participants } = event;

  const beginning_of_day = start_time.startOf("day");
  const end_of_day = start_time.endOf("day");
  const milliseconds_in_time_period = end_of_day.diff(beginning_of_day);

  // Keep track of the height as it gets updated after the ref gets set
  const [container_height, set_container_height] = useState(0);

  // Reset the height on window resize
  useWindowResize(() => {
    set_container_height(container_ref.current.clientHeight);
  })

  /** A function to convert a date object into a time string
  */
  function display_time(date) {
    return date.format("HH:mm");
  }

  /** Linearly interpolates n that is between min_n and max_n into the range between min_out and max_out
  */
  function interpolate(n, min_n, max_n, min_out, max_out) {
    return min_out + (n - min_n) / (max_n - min_n) * max_out;
  }


  return <div style={{
    top: `${interpolate(start_time.valueOf(), beginning_of_day.valueOf(), end_of_day.valueOf(), 0, container_height)}px`,// interpolate between the beginning and end of day
    height: `${interpolate(duration, 0, milliseconds_in_time_period, 0, container_height)}px`,// interpolate between the beginning and end of day
  }}
    className="position-absolute border w-100">
    <em className="fs-6 text-truncate">{title}</em>
    <div className="fs-6 text-truncate">{participants}</div>
    <div className="fs-6 text-truncate">{display_time(start_time)}-{display_time(end_time)}</div>
  </div>
}
