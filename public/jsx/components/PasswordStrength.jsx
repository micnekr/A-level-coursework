/** A component that represents a single bar in a PasswordStrength component
 */
function PasswordStrengthBar(props){
  const {children, color} = props;
  // Pass on the additional classes
  return <div
           className={`col-2 rounded-pill border mx-1`}
           style={{
             height: "1rem",
             backgroundColor: color
           }}
         />
}

/** A component that calculates password strength and
 * displays a number of bars proportional to the password strength
 */
function PasswordStrength(props) {
  const { password} = props;

  // Do not display anything on empty passwords
  if (password === "") return null;

  // Bar colours based on password strength
  const bar_colors = [
    "#F71735", // red
    "#FF7700", // orange
    "#FF7700", // orange
    "#A1E887", // green
    "#A1E887", // green
  ];

  // Analyse the password strength
  const password_analysis_result = zxcvbn(password);

  // the score goes from 0 to 4
  const password_strength = password_analysis_result.score;
  const password_strength_message = password_analysis_result.feedback.warning;

  const bar_color = bar_colors[password_strength];

  // Note that the number of highlighted bars is the same as the password strength
  const total_bar_num = 5;
  // Always highlight at least one bar. Note that password strength varies between 0 and 4 (inclusive)
  const num_bars_highlighted = password_strength + 1;
  const num_bars_not_highlighted = total_bar_num - num_bars_highlighted;

  return <div className="container-fluid">

           <div className="row justify-content-center mb-3">
                {/* Create `password_strength` highlighted rows */}
                {Array(num_bars_highlighted).fill().map((_, i) =>
                    <PasswordStrengthBar key={i} color={bar_color}/>
                )}

                {/* Create the non-highlighted rows to get a total of `total_bar_num` */}
                {Array(num_bars_not_highlighted).fill().map((_, i) =>
                    <PasswordStrengthBar key={i}/>
                )}
           </div>
            {
                // if it an empty string, do not display it
            password_strength_message === "" ? null:
                <div className="text-center mb-3">
                Warning: {password_strength_message}
                </div>
            }
  </div>
}
