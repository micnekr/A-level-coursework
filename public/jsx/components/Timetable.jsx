const useRef = React.useRef;

/** A component that displays a timetable
 */
function Timetable(props) {
  // date is any date in this week
  const { events: raw_events, start_of_week_date } = props;

  // Process the events
  raw_events.forEach(element => {
    // Convert so that it can be used with the library
    element.start_time = dayjs(element.start_time);
    // Calculate the time the event ends at
    element.end_time = element.start_time.add(element.duration, "milliseconds");
  });

  // If an event runs over to the next day, split it into two.
  let events = [];
  raw_events.forEach(element => {
    if (element.start_time.dayOfYear() !== element.end_time.dayOfYear()) {
      // Create two events and push them.
      const event1 = { ...element };
      const event2 = { ...element };

      // Reset the times to the end and beginning of those days
      event1.end_time = element.start_time.endOf("day");
      event2.start_time = element.end_time.startOf("day");
      // Also reset the durations
      event1.duration = event1.end_time.diff(event1.start_time);
      event2.duration = event2.end_time.diff(event2.start_time);

      events.push(event1);
      events.push(event2);
    } else
      events.push(element);
  });

  // References to the 7 columns
  const div_timetable_column_refs = Array(7).fill().map(() => useRef(null));

  return <div className="container-lg border p-1">
    <div className="row gx-0">
      {/* Add 7 columns for each day of the week*/}
      {
        Array(7).fill().map((_, i) => {
          const day = start_of_week_date.add(i, "day");
          return <div className="col-md" key={i} >
            <h5 className="text-center">
              {display_day_and_date(day)}
            </h5>
            {/* A column corresponding to events on one day*/}
            <div className="border border-light position-relative" style={{
              height: "1400px",
              backgroundColor: "#f5f5f5"
            }} ref={div_timetable_column_refs[i]}>

              {
                /* Filter out only the events from this day */
                events.filter(el => el.start_time.day() === day.day()).map((event, i) =>
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
