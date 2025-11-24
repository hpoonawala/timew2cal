## Timew2Cal 

Converts the timewarrior data into a csv that you can import into Google Calendar. 

- Go to  `/path/to/timew2cal`
- Generate a report:
    ```
    timew export :week > input.json
    ```
- Run
    ```
    ./target/release/timew2cal input.json out.csv`
    ```
- In google calendar, use settings to import `out.csv`
    
