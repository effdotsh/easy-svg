
      const consoleProxy = new Proxy(console, {
        get(target, prop) {
          if (typeof target[prop] === "function") {
            return (...args) => {
              try {
                window.parent.postMessage({ typ: "console", prop, args }, "*");
              } catch {
                try {
                  window.parent.postMessage(
                    {
                      typ: "console",
                      prop,
                      args: args.map((x) => {
                        try {
                          window.structuredClone(x);
                          return x;
                        } catch {
                          return {
                            _MDNPlaySerializedObject: x.toString(),
                          };
                        }
                      }),
                    },
                    "*"
                  );
                } catch {
                  window.parent.postMessage(
                    {
                      typ: "console",
                      prop: "warn",
                      args: [
                        "[Playground] Unsupported console message (see browser console)",
                      ],
                    },
                    "*"
                  );
                }
              }
              target[prop](...args);
            };
          }
          return target[prop];
        },
      });

      window.console = consoleProxy;
      window.addEventListener("error", (e) => console.log(e.error));
    