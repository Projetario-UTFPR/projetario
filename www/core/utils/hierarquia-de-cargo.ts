import type { Cargo } from "../types/enums/cargo";

export function cargoEhMaiorOuIgual(cargo: Cargo, expectativa: Cargo): boolean {
  if (expectativa === "Aluno") return cargo === "Aluno";

  if (expectativa === "Administrador") return cargo === "Administrador";

  if (expectativa === "Professor")
    return ["Professor", "Administrador"].includes(cargo);

  throw new Error(
    "Chamou `cargoEhMaiorOuIgual` com um cargo esperado inválido: " +
      expectativa,
  );
}
