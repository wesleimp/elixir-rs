defmodule ElixirRS.MixProject do
  use Mix.Project

  @version "0.1.0"

  def project do
    [
      app: "elixir_rs",
      version: @version,
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      elixir: "~> 1.13",
      aliases: aliases(),
      releases: releases()
    ]
  end

  defp deps do
    [
      {:phoenix, "~> 1.5.4"},
      {:phoenix_ecto, "~> 4.0"},
      {:phoenix_html, "~> 2.11"},
      {:phoenix_live_reload, "~> 1.2", only: :dev},
      {:phoenix_live_dashboard, "~> 0.2"},
      {:phoenix_live_view, "~> 0.14.8"},
      {:floki, ">= 0.27.0", only: :test},
      {:telemetry_metrics, "~> 0.4"},
      {:telemetry_poller, "~> 0.4"},
      {:gettext, "~> 0.11"},
      {:logger_json, "~> 4.0"},
      {:cowlib, "~> 2.10", override: true},
      {:gun, "~> 2.0.0", hex: :grpc_gun, override: true},
      {:credo, "~> 1.4", only: [:dev, :test], runtime: false},
      {:credo_sonarqube, "~> 0.1.0", only: [:test, :dev]},
      {:mock, "~> 0.3.0", only: :test},
      {:new_relic_agent, "~> 1.22"},
      {:bakeware, "~> 0.2", runtime: false}
    ]
  end

  defp aliases do
    [
      setup: ["cmd mix setup"]
    ]
  end

  defp releases() do
    [
      overwrite: true,
      include_executables_for: [:unix],
      applications: [
        runtime_tools: :permanent,
        elixir_rs: :permanent,
      ],
      steps: [
        :assemble,
        &Bakeware.assemble/1
      ],
      bakeware: [compression_level: 19]
    ]
  end
end
