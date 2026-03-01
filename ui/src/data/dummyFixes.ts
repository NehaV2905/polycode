export const dummyFixes = {
  repo: "https://github.com/NehaV2905/Task-Manager.git",
  files_parsed: 16,
  total_findings: 33,
  cap: 10,
  suggestions: [
    {
      id: 1,
      file: "org/example/TodoApplication.java",
      line: 9,
      function: "main",
      suggestion: "The main function is the entry point of a Java application, so it's unlikely to be dead code if the application is running successfully. Verify that the application can be started from the command line using this main method. If the application is a Spring Boot application, this main method is necessary and should not be deleted."
    },
    {
      id: 2,
      file: "org/example/config/JwtFilter.java",
      line: 29,
      function: "doFilterInternal",
      suggestion: "The doFilterInternal function appears to be a crucial part of the JwtFilter class, which extends OncePerRequestFilter. Consider adding a call-site or checking the filter's configuration to ensure it's properly registered and enabled."
    },
    {
      id: 3,
      file: "org/example/config/SecurityConfig.java",
      line: 33,
      function: "securityFilterChain",
      suggestion: "The securityFilterChain function is a crucial configuration component for Spring Security and should not be deleted. Since it's annotated with @Bean, it will be automatically registered by Spring, making explicit calls unnecessary."
    },
    {
      id: 4,
      file: "org/example/config/SecurityConfig.java",
      line: 58,
      function: "authenticationProvider",
      suggestion: "The authenticationProvider function is not being used anywhere. Consider deleting it entirely or refactoring it into the authenticationManager function to configure the AuthenticationManager with the DaoAuthenticationProvider instance."
    },
    {
      id: 5,
      file: "org/example/config/SecurityConfig.java",
      line: 68,
      function: "authenticationManager",
      suggestion: "The authenticationManager function can be deleted entirely, as it is not being used anywhere and its functionality can be achieved directly through the AuthenticationConfiguration object."
    },
    {
      id: 6,
      file: "org/example/controller/TodoController.java",
      line: 34,
      function: "create",
      suggestion: "The create function is not dead code as it is being called via the @PostMapping('/add') endpoint. The issue might be with the static analysis tool. Consider adding a test case to ensure the endpoint is working as expected."
    },
    {
      id: 7,
      file: "org/example/controller/UserController.java",
      line: 21,
      function: "login",
      suggestion: "The login function is flagged as dead code but it's intended to be an entry-point for user authentication via @PostMapping('/login'). Leave it in place and verify it's properly accessible from the intended caller."
    },
    {
      id: 8,
      file: "org/example/model/Todo.java",
      line: 39,
      function: "hashCode",
      suggestion: "The hashCode function is a crucial override when equals is also overridden. Verify that Todo instances are used in hash-based data structures like HashMap. If so, keep it. If not, it could be deleted."
    },
    {
      id: 9,
      file: "org/example/model/Todo.java",
      line: 44,
      function: "toString",
      suggestion: "The toString function is a standard method often used for debugging. Consider whether it's necessary; if you anticipate needing to debug or log Todo objects in the future, it might be worth keeping."
    },
    {
      id: 10,
      file: "org/example/model/UserPrincipal.java",
      line: 18,
      function: "getAuthorities",
      suggestion: "The getAuthorities function is an overridden method from the UserDetails interface and is crucial for Spring Security. Verify that the UserPrincipal class is being used correctly in the application's security configuration."
    },
  ]
};