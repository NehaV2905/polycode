export const dummyIR = {
  projectName: "PolyCode Analyzer",
  languages: ["Python", "Java", "Go"],

  modules: [
    {
      name: "auth.py",
      language: "Python",
      functions: ["login", "hashPassword"]
    },
    {
      name: "UserService.java",
      language: "Java",
      functions: ["authenticate"]
    },
    {
      name: "handler.go",
      language: "Go",
      functions: ["LoginHandler"]
    }
  ],

  functions: [
    {
      name: "login",
      module: "auth.py",
      calls: ["hashPassword"],
      returns: "bool"
    },
    {
      name: "hashPassword",
      module: "auth.py",
      calls: [],
      returns: "string"
    },
    {
      name: "authenticate",
      module: "UserService.java",
      calls: ["login"],
      returns: "boolean"
    },
    {
      name: "LoginHandler",
      module: "handler.go",
      calls: ["authenticate"],
      returns: "Response"
    }
  ]
};
