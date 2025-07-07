import Logo from "@/assets/logo-projetario.svg";
import { BotaoDeLogin } from "./botao-de-login";
import { BotaoDeTema } from "./botao-de-tema";
import { DropdownUsuario } from "./dropdown-usuario";
import { NavItem } from "./nav-item";

export function Header() {
  return (
    <header className="bg-gray-100 py-2.5 border-b border-black/10">
      <div className="w-[calc(100%_-_48px)] max-w-lg mx-auto flex items-center justify-between">
        <img src={Logo} alt="Logo Projetário" width={216} height={50} />

        <div className="flex gap-6 divide-x divide-black/10 dark:divide-white/10">
          <nav className="flex items-stretch pr-6">
            <NavItem href="/" label="Início" />
            <NavItem href="/projetos" label="Projetos" />
          </nav>

          <div className="flex items-center gap-2">
            <BotaoDeTema />
            <BotaoDeLogin />
            <DropdownUsuario />
          </div>
        </div>
      </div>
    </header>
  );
}
