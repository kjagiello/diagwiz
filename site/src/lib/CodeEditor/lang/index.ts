import { parser } from "./syntax.grammar";
import { LezerLanguage, LanguageSupport } from "@codemirror/language";
import { styleTags, tags as t } from "@codemirror/highlight";

export const DiagwizLanguage = LezerLanguage.define({
  parser: parser.configure({
    props: [
      styleTags({
        LineComment: t.lineComment,
        "Alias/alias": t.definitionKeyword,
        "Alias/Identifier": t.variableName,
        "Alias/String": t.string,
        "Message/Identifier": t.variableName,
        "Message/String": t.string,
      }),
    ],
  }),
});

export function diagwiz(): LanguageSupport {
  return new LanguageSupport(DiagwizLanguage);
}
