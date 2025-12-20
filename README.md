# Courses Calculator

The goal of this project is to develop a web application that facilitates the process of calculating the advised
course length range for an orienting course.

The ideal range is determined by analyzing the results of selected stages from prior events. Such results are
fetched from [OriOasis](https://www.orioasis.pt).

Another main goal is to learn new technologies and frameworks during the development of this project.

The end goal would be to deploy the application on a cloud platform and automate the most as possible that process.

# Functional Requirements

1. The user must be able to create a Cource Calculator Session, providing a name and a description.
2. The system must allow to search on the history of Orioasis events.
3. Once events are found, the user should be able to select stages within those events.
4. Stages must be able to be attached into a Course Calculator Session.
5. Once all stages are selected, the user must be able to request the fetch of data from orioasis
   of the stages attached to the Course Calculator Session.
6. Once all stages are imported, the system should allow the user to request approximate stage duration times
   by providing the minimum and maximum distances. Two approximate values should be returned. One for the fastest runner
   and the other for the fastest portuguese runner.
