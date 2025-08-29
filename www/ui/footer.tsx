import { type InertiaLinkProps, Link, usePage } from "@inertiajs/react";
import { GithubLogoIcon } from "@phosphor-icons/react/dist/ssr/GithubLogo";
import { InstagramLogoIcon } from "@phosphor-icons/react/dist/ssr/InstagramLogo";
import { LinkedinLogoIcon } from "@phosphor-icons/react/dist/ssr/LinkedinLogo";
import { XLogoIcon } from "@phosphor-icons/react/dist/ssr/XLogo";
import clsx from "clsx";
import Logo from "@/assets/logo-projetario.svg";
import { Hr } from "@/components/hr";
import { RedesSociais } from "@/core/redes-sociais";

export function Footer() {
  const pageProps = usePage().props;
  const usuarioEstaLogado = !!pageProps.autenticacao;
  const usuarioEhProfessor =
    usuarioEstaLogado && pageProps.autenticacao!.usuario.cargo !== "Aluno";

  return (
    <footer className="bg-gray-200 dark:bg-white/5 px-6 py-12 prose-h6:cursor-default">
      <div className="mx-auto w-[calc(100%_-_48px)] max-w-lg flex gap-36">
        <div className="flex flex-col gap-6">
          <figure>
            <img src={Logo} width={432} height={100} alt="Projetário" />
            <figcaption className="uppercase font-bold cursor-default">
              Centro de Projetos e Pesquisas da UTFPR
            </figcaption>
          </figure>

          <Hr />

          <div className="flex gap-8">
            <div>
              <h6 className="mb-2 uppercase">Redes do Projetário</h6>
              <div className="flex gap-2">
                <a
                  target="_blank"
                  referrerPolicy="no-referrer"
                  href={RedesSociais.projetario.github}
                >
                  <GithubLogoIcon size={24} weight="fill" />
                </a>
                <a
                  target="_blank"
                  referrerPolicy="no-referrer"
                  href={RedesSociais.projetario.instagram}
                >
                  <InstagramLogoIcon size={24} weight="fill" />
                </a>
              </div>
            </div>

            <div>
              <h6 className="mb-2 uppercase">Redes da UTFPR</h6>
              <div className="flex gap-2">
                <a
                  target="_blank"
                  referrerPolicy="no-referrer"
                  href={RedesSociais.utfpr.instagram}
                >
                  <InstagramLogoIcon size={24} weight="fill" />
                </a>
                <a
                  target="_blank"
                  referrerPolicy="no-referrer"
                  href={RedesSociais.utfpr.linkedin}
                >
                  <LinkedinLogoIcon size={24} weight="fill" />
                </a>
                <a
                  target="_blank"
                  referrerPolicy="no-referrer"
                  href={RedesSociais.utfpr.x}
                >
                  <XLogoIcon size={24} weight="fill" />
                </a>
              </div>
            </div>
          </div>
        </div>

        <div
          className={clsx(
            "flex-1 flex gap-12 prose-a:text-lg",
            "prose-h6:text-black/75 prose-h6:dark:text-white/75 prose-h6:uppercase prose-h6:text-base",
          )}
        >
          <div className="flex-1 flex flex-col gap-2">
            <h6>Recursos</h6>
            <FooterLink>FAQ</FooterLink>
            {!usuarioEstaLogado && (
              <FooterLink href="/autenticacao/login">Login</FooterLink>
            )}
            {usuarioEhProfessor && (
              <FooterLink href="#">Painel do professor</FooterLink>
            )}
            <FooterLink>Termos e condições</FooterLink>
          </div>

          <div className="flex-1 flex flex-col gap-2">
            <h6>Projetos</h6>
            <FooterLink disabled>Projetos de extensão</FooterLink>
            <FooterLink disabled>Pesquisas de iniciação científica</FooterLink>
            <FooterLink disabled>Mentores de TCC</FooterLink>
          </div>
        </div>
      </div>
    </footer>
  );
}

function FooterLink({
  children,
  className,
  href = "#",
  disabled,
  ...props
}: InertiaLinkProps) {
  return (
    <Link
      href={href}
      className={clsx(
        "text-lg decoration-wavy decoration-1 underline decoration-transparent",
        "transition-all duration-100",
        disabled
          ? "opacity-50 cursor-not-allowed"
          : "hover:decoration-black/50 dark:hover:decoration-white/50 active:opacity-80",
        className && className,
      )}
      {...props}
    >
      {children}
    </Link>
  );
}
