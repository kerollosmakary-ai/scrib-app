# Scrib Bricks

Bricks are small, observable capabilities. Each brick has a clear input, a check, and a result. A later brick must not assume that an earlier brick succeeded.

## Bootstrap order

1. **Core** — start the Scrib UI and persistent note store.
2. **Clipboard** — read and write clipboard text through the host integration.
3. **AI** — turn natural-language editing requests into text transformations.
4. **Command Plan** — turn a Linux request into a proposed command sequence.
5. **Validation** — reject malformed or disallowed commands before execution.
6. **Alpine** — prepare the Alpine userspace and PRoot runtime.
7. **Shell Session** — start and attach to an Alpine shell.
8. **Feedback** — capture stdout/stderr and return failures to the command planner.

## Rule

A brick reports success only after its own check succeeds. AI output is data until the validation brick approves it. Shell execution is never the validation step.

## Educational mode

The UI should be able to show what each brick is doing. This makes the Linux environment understandable instead of hiding the system behind an AI abstraction.
