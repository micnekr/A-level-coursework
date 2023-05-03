/** A component that calculates password strength and
 * displays a progress bar that represents the password strength
 */
function PasswordStrength(props) {
    const { password } = props;

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

    const total_bar_length = 5;
    // Always show a little progress. Note that password strength varies between 0 and 4 (inclusive)
    const length_highlighted = password_strength + 1;

    return <div className="container-fluid">

        <div className="row mb-3">
            <div className="col-auto ps-0 mb-2 mb-sm-0">Password strength: </div>
            {/* The "outside" of the bar */}
            <div className="col-sm rounded-pill border px-0 mx-auto align-self-center" style={{
                height: "1rem"
            }}>
                {/* The colourful "inside" of the bar */}
                <div className="h-100 rounded-pill" style={{
                    backgroundColor: bar_color,
                    // Calculate percentage width
                    width: `${length_highlighted / total_bar_length * 100}%`
                }}></div>
            </div>
        </div>
        {
            // if it an empty string, do not display it
            password_strength_message === "" ? null :
                <div className="text-center mb-3">
                    Warning: {password_strength_message}
                </div>
        }
    </div>
}
