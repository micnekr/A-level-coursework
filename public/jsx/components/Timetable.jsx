const useRef = React.useRef;

/** A component that displays a timetable
 */
function Timetable(props) {
  const days_of_the_week = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
  const { events: raw_events, date } = props;

  const initial_date = dayjs(date).startOf("week").add(1, "day"); // Their week starts on a Sunday

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

  return <div className="container border p-1">
    <div className="row gx-0">
      {/* Add 7 columns for each day of the week*/}
      {
        Array(7).fill().map((_, i) => {
          const day = initial_date.add(i, "day");
          return <div className="col" key={i}>
            <h4 className="text-center">
              {days_of_the_week[i]}
              {" "}
              {to_ordinal(day.date())}
            </h4>
            {/* A column corresponding to events on one day*/}
            <div className="border position-relative" style={{
              height: "800px"
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
  </div>
}
