const useRef = React.useRef;

/** A component that displays a timetable
 */
function Timetable(props) {
  // date is any date in this week
  const { events: raw_events, start_of_week_date } = props;
  const end_of_week_date = start_of_week_date.endOf("week");

  // If an event runs over to the next day, split it into two.
  let events = [];
  raw_events.forEach(element => {
    // Convert so that it can be used with the library
    const start_time = dayjs.unix(element.start_time);
    // Calculate the time the event ends at
    const end_time = start_time.add(element.duration, "seconds");

    // Check if the event runs over midnight and therefore is split over two days
    if (start_time.dayOfYear() !== end_time.dayOfYear()) {
      // Create two events and push them.
      const event1 = { ...element };
      const event2 = { ...element };

      // Reset the times to the end and beginning of those days
      event1.start_time = start_time;
      event1.end_time = start_time.endOf("day");
      event2.start_time = end_time.startOf("day");
      event2.end_time = end_time;
      // Also reset the durations
      event1.duration = event1.end_time.unix() - event1.start_time.unix();
      event2.duration = event2.end_time.unix() - event2.start_time.unix();

      events.push(event1);
      events.push(event2);
    } else {
      // Copy the event fields to avoid modifying the original element
      const event = { ...element };
      event.start_time = start_time;
      event.end_time = end_time;

      events.push(event);
    }
  });

  // Make sure that each event has a duration of at least a minute to avoid displaying "empty" events
  events = events.filter(event => event.duration > 59);

  // References to the 7 columns
  const div_timetable_column_refs = Array(7).fill().map(() => useRef(null));

  return <div className="container-lg border p-1">
    <div className="row gx-0">
      {/* Add 7 columns for each day of the week*/}
      {
        Array(7).fill().map((_, i) => {
          const day_in_this_column = start_of_week_date.add(i, "day");
          return <div className="col-md" key={i} >
            <h5 className="text-center">
              {display_day_and_date(day_in_this_column)}
            </h5>
            {/* A column corresponding to events on one day*/}
            <div className="border border-light position-relative" style={{
              height: "1400px",
              backgroundColor: "#f5f5f5"
            }} ref={div_timetable_column_refs[i]}>

              {
                /* Filter out only the events from this day */
                events.filter(el => {
                  // Check if it is a weekly event
                  if (el.recurrence_type === "Weekly") {
                    // check if the day of the week is the same
                    const is_same_day_of_the_week = el.start_time.day() === day_in_this_column.day()

                    // Check if it is not before the first time the event has occured
                    const is_at_least_first_time = el.start_time.unix() <= end_of_week_date.unix();

                    return is_same_day_of_the_week && is_at_least_first_time;
                  } else {
                    // Check that the day is the same by checking that the "beginning of the day" refers to the same day
                    return el.start_time.startOf("day").unix() === day_in_this_column.startOf("day").unix();
                  }
                }
                ).map((event, i) =>
                  <TimetableEvent key={i} event={event} container_ref={div_timetable_column_refs[i]} />
                )
              }
            </div>
          </div>
        }
        )
      }
    </div>
  </div >
}
