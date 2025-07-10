import { type FormEvent, useState } from "react";
import { FaSearch } from "react-icons/fa";

const Pesquisar = () => {
  const [filtro, setFiltro] = useState("");
  const [tipoSelecionado, setTipoSelecionado] = useState("extensao");
  const [ordenacaoSelecionada, setOrdenacaoSelecionada] = useState("Data(a-z)");

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    console.log({
      filtro,
      tipo: tipoSelecionado,
      ordenacao: ordenacaoSelecionada,
    });
  };

  return (
    <div className="min-h-screen bg-gray-100 flex items-center justify-center p-4">
      <div className="w-full max-w-4xl bg-white rounded-lg shadow-md p-6">
        <h1 className="text-2xl font-bold text-gray-800 mb-2">
          Projetos e Pesquisas
        </h1>
        <p className="text-gray-600 mb-6">
          Encontre projetos em nossa base de dados
        </p>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="flex flex-col sm:flex-row gap-4 items-center">
            {/* Campo de busca */}
            <div className="relative flex-grow">
              <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <FaSearch className="text-gray-400" />
              </div>
              <input
                type="text"
                placeholder="Encontre qualquer projeto"
                className="block w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                value={filtro}
                onChange={(e) => setFiltro(e.target.value)}
              />
            </div>

            {/* Botão de pesquisa */}
            <button
              type="submit"
              className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors flex items-center"
            >
              <FaSearch className="mr-2" />
              Pesquisar
            </button>
          </div>

          <div className="flex flex-wrap gap-4">
            {/* Filtros: Extensão e Iniciação Científica */}
            <div className="flex space-x-2">
              {["Extensão", "Iniciação Científica"].map((tipo) => (
                <button
                  key={tipo}
                  type="button"
                  className={`px-3 py-1 rounded-full text-sm ${
                    tipoSelecionado === tipo.toLowerCase().replace(" ", "_")
                      ? "bg-blue-100 text-blue-800"
                      : "bg-gray-100 text-gray-700 hover:bg-gray-200"
                  }`}
                  onClick={() =>
                    setTipoSelecionado(tipo.toLowerCase().replace(" ", "_"))
                  }
                >
                  {tipo}
                </button>
              ))}
            </div>

            {/* Ordenação e Contador */}
            <div className="flex items-center space-x-2 ml-auto">
              <div className="relative">
                <select
                  className="px-3 py-1 bg-gray-100 rounded-full text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  value={ordenacaoSelecionada}
                  onChange={(e) => setOrdenacaoSelecionada(e.target.value)}
                >
                  <option value="Data(a-z)">Data (a-z)</option>
                  <option value="Data(z-a)">Data (z-a)</option>
                  <option value="Título(a-z)">Título (a-z)</option>
                  <option value="Título(z-a)">Título (z-a)</option>
                </select>
              </div>

              <div className="bg-blue-500 text-white rounded-full px-2 py-1 text-xs font-medium">
                88
              </div>
            </div>
          </div>
        </form>

        {/* Resultados simplificados */}
        <div className="mt-8">
          <h2 className="text-lg font-semibold text-gray-800 mb-4">
            Projetos Encontrados
          </h2>
          <div className="space-y-3">
            {[1, 2, 3].map((item) => (
              <div
                key={item}
                className="p-4 bg-gray-50 rounded-lg border border-gray-200"
              >
                <h3 className="font-medium text-gray-800">
                  Projeto de Exemplo {item}
                </h3>
                <p className="text-sm text-gray-600 mt-1">
                  Descrição breve do projeto de pesquisa
                </p>
                <div className="flex justify-between mt-2">
                  <span className="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded">
                    {item % 2 === 0 ? "Extensão" : "Iniciação Científica"}
                  </span>
                  <span className="text-xs text-gray-500">10/07/2025</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default Pesquisar;
