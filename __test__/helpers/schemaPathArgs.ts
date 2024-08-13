import path from 'path'

export default function schemaPathArgs(file: string, schemaFileName: string): string[] {
    const schemaFilePath = path.join(path.dirname(file), schemaFileName)
    return ["teo", "serve", "--schema", schemaFilePath]
}