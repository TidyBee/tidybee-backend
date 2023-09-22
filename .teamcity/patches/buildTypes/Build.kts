package patches.buildTypes

import jetbrains.buildServer.configs.kotlin.*
import jetbrains.buildServer.configs.kotlin.buildFeatures.PullRequests
import jetbrains.buildServer.configs.kotlin.buildFeatures.commitStatusPublisher
import jetbrains.buildServer.configs.kotlin.buildFeatures.pullRequests
import jetbrains.buildServer.configs.kotlin.buildSteps.DotnetBuildStep
import jetbrains.buildServer.configs.kotlin.buildSteps.dockerCommand
import jetbrains.buildServer.configs.kotlin.buildSteps.dotnetBuild
import jetbrains.buildServer.configs.kotlin.buildSteps.script
import jetbrains.buildServer.configs.kotlin.ui.*

/*
This patch script was generated by TeamCity on settings change in UI.
To apply the patch, change the buildType with id = 'Build'
accordingly, and delete the patch script.
*/
changeBuildType(RelativeId("Build")) {
    vcs {
        remove(DslContext.settingsRoot.id!!)
        add(RelativeId("HttpsGithubComTidyBeeTidybeeBackendRefsHeadsMain3"))
    }

    expectSteps {
        dotnetBuild {
            name = "Build"
            projects = """
                functionnalTests/*.csproj
                unitaryTests/*.csproj
            """.trimIndent()
            logging = DotnetBuildStep.Verbosity.Normal
            dockerImage = "mcr.microsoft.com/dotnet/sdk:7.0"
            param("dotNetCoverage.dotCover.home.path", "%teamcity.tool.JetBrains.dotCover.CommandLineTools.DEFAULT%")
        }
        script {
            name = "Tests"
            scriptContent = """
                cd functionnalTests
                dotnet publish -o out
                dotnet vstest out/TidyUpSoftware.xUnitTests.dll
                cd ..
                dotnet publish -o out
                dotnet vstest out/TidyUpSoftware.nUnitTests.dll
            """.trimIndent()
        }
    }
    steps {
        insert(0) {
            dockerCommand {
                commandType = build {
                    source = file {
                        path = "Dockerfile"
                    }
                }
            }
        }
        items.removeAt(1)
        items.removeAt(1)
    }

    features {
        remove {
            pullRequests {
                provider = github {
                    authType = vcsRoot()
                    filterAuthorRole = PullRequests.GitHubRoleFilter.MEMBER
                    ignoreDrafts = true
                }
            }
        }
        remove {
            commitStatusPublisher {
                publisher = github {
                    githubUrl = "https://api.github.com"
                    authType = personalToken {
                        token = "credentialsJSON:0f816045-3db7-4a38-893d-b59e0b71a889"
                    }
                }
                param("github_oauth_user", "Cavonstavant")
            }
        }
    }
}
